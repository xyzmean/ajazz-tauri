const { app, BrowserWindow } = require('electron');
const path = require('path');
const http = require('http');
const fs = require('fs');

let server;

// Supported MIME types for our local static server
const MIME_TYPES = {
  '.html': 'text/html',
  '.css': 'text/css',
  '.js': 'text/javascript',
  '.json': 'application/json',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.gif': 'image/gif',
  '.ico': 'image/x-icon',
  '.svg': 'image/svg+xml'
};

// Create a lightweight, 100% offline static file server
function startLocalServer(callback) {
  server = http.createServer((req, res) => {
    let filePath = req.url.split('?')[0];
    if (filePath === '/') {
      filePath = '/index.html';
    }

    const fullPath = path.join(__dirname, 'app', filePath);

    if (!fullPath.startsWith(path.join(__dirname, 'app'))) {
      res.statusCode = 403;
      res.end('Forbidden');
      return;
    }

    const ext = path.extname(fullPath).toLowerCase();
    const contentType = MIME_TYPES[ext] || 'application/octet-stream';

    fs.readFile(fullPath, (err, data) => {
      if (err) {
        if (err.code === 'ENOENT') {
          res.statusCode = 404;
          res.end('Not Found');
        } else {
          res.statusCode = 500;
          res.end('Internal Server Error');
        }
      } else {
        res.writeHead(200, { 
          'Content-Type': contentType,
          'Access-Control-Allow-Origin': '*'
        });
        res.end(data);
      }
    });
  });

  server.listen(0, '127.0.0.1', () => {
    const port = server.address().port;
    console.log(`Local offline server running at http://127.0.0.1:${port}`);
    callback(port);
  });
}

function createWindow(port) {
  const win = new BrowserWindow({
    width: 1440,
    height: 900,
    title: "AJAZZ Local Configuration Utility (Offline)",
    autoHideMenuBar: true,
    show: false, // Prevent white flicker on show
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      webSecurity: false, // Crucial for WebHID and local caching
      zoomFactor: 0.85   // Scales down elements to fit smaller/laptop displays beautifully!
    }
  });

  // Start maximized
  win.maximize();
  win.show();

  win.loadURL(`http://127.0.0.1:${port}/`);

  // Forward console messages from browser to terminal
  win.webContents.on('console-message', (event, level, message, line, sourceId) => {
    const levels = ['DEBUG', 'INFO', 'WARN', 'ERROR'];
    console.log(`[BROWSER CONSOLE][${levels[level] || 'LOG'}] ${message} (at ${path.basename(sourceId)}:${line})`);
  });

  // Only open DevTools if launched with '--devtools' flag
  const isDebug = process.argv.includes('--devtools');
  if (isDebug) {
    win.webContents.openDevTools();
  }

  // WebHID Integration: Automatically grant access to connected USB keyboards
  win.webContents.session.on('select-hid-device', (event, details, callback) => {
    event.preventDefault();
    if (details.deviceList && details.deviceList.length > 0) {
      console.log('WebHID request received. Auto-selecting device:', details.deviceList[0].name);
      callback(details.deviceList[0].deviceId);
    } else {
      console.log('WebHID request received, but no HID devices were found.');
      callback();
    }
  });

  win.webContents.session.setPermissionCheckHandler((webContents, permission, requestingOrigin, details) => {
    if (permission === 'hid') {
      return true;
    }
    return false;
  });

  win.webContents.session.setDevicePermissionHandler((details) => {
    if (details.deviceType === 'hid') {
      return true;
    }
    return false;
  });
}

app.whenReady().then(() => {
  startLocalServer((port) => {
    createWindow(port);
  });

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      startLocalServer((port) => {
        createWindow(port);
      });
    }
  });
});

app.on('window-all-closed', () => {
  if (server) {
    server.close();
  }
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
