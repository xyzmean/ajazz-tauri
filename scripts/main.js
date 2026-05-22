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
    // Parse the requested file path (remove query params)
    let filePath = req.url.split('?')[0];
    if (filePath === '/') {
      filePath = '/index.html';
    }

    // Form absolute file path inside the 'app' directory
    const fullPath = path.join(__dirname, 'app', filePath);

    // Security check: ensure the request stays within the 'app' folder
    if (!fullPath.startsWith(path.join(__dirname, 'app'))) {
      res.statusCode = 403;
      res.end('Forbidden');
      return;
    }

    // Determine correct Content-Type header
    const ext = path.extname(fullPath).toLowerCase();
    const contentType = MIME_TYPES[ext] || 'application/octet-stream';

    // Read and serve the local file
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
          'Access-Control-Allow-Origin': '*' // Add CORS headers for absolute local loading
        });
        res.end(data);
      }
    });
  });

  // Listen on a random available port on the local loopback (127.0.0.1)
  server.listen(0, '127.0.0.1', () => {
    const port = server.address().port;
    console.log(`Local offline server running at http://127.0.0.1:${port}`);
    callback(port);
  });
}

function createWindow(port) {
  const win = new BrowserWindow({
    width: 1280,
    height: 800,
    title: "AJAZZ Local Configuration Utility (Offline)",
    autoHideMenuBar: true, // Clean, modern design
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      webSecurity: false // Necessary for local file access and WebHID requests
    }
  });

  // Load the app via our local micro-server (resolves all absolute "/assets/..." pathing issues)
  win.loadURL(`http://127.0.0.1:${port}/`);

  // Forward console messages from browser to Node.js terminal for debugging
  win.webContents.on('console-message', (event, level, message, line, sourceId) => {
    const levels = ['DEBUG', 'INFO', 'WARN', 'ERROR'];
    console.log(`[BROWSER CONSOLE][${levels[level] || 'LOG'}] ${message} (at ${path.basename(sourceId)}:${line})`);
  });

  // Open Developer Tools by default for troubleshooting
  win.webContents.openDevTools();

  // WebHID Integration: Automatically grant access to connected USB keyboards
  win.webContents.session.on('select-hid-device', (event, details, callback) => {
    event.preventDefault();
    if (details.deviceList && details.deviceList.length > 0) {
      console.log('WebHID request received. Auto-selecting device:', details.deviceList[0].name);
      callback(details.deviceList[0].deviceId);
    } else {
      console.log('WebHID request received, but no HID devices were found.');
      callback(); // Cancel if no device is connected
    }
  });

  // Grant global 'hid' permission check
  win.webContents.session.setPermissionCheckHandler((webContents, permission, requestingOrigin, details) => {
    if (permission === 'hid') {
      return true;
    }
    return false;
  });

  // Approve all HID device types
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
  // Shut down local server
  if (server) {
    server.close();
  }
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
