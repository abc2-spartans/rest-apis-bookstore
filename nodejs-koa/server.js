const Koa = require('koa');
const Router = require('koa-router');
const bodyParser = require('koa-bodyparser');
const sqlite3 = require('sqlite3').verbose();
const path = require('path');

const app = new Koa();
const router = new Router();
const PORT = process.env.PORT || 5000;

// Middleware
app.use(bodyParser());

// SQLite DB setup
const dbPath = path.join(__dirname, 'bookstore.db');
const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error('Failed to connect to database:', err.message);
  } else {
    console.log('Connected to the SQLite database.');
  }
});

db.serialize(() => {
  db.run(`CREATE TABLE IF NOT EXISTS books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    published_year INTEGER
  )`);
});

// Health check
router.get(['/', '/api/v1', '/health'], async (ctx) => {
  ctx.body = {
    status: 'healthy',
    timestamp: new Date().toISOString(),
    service: 'Bookstore API',
    apiURL: `http://localhost:${PORT}/api/v1/books`
  };
});

// CRUD endpoints for books
router.get('/api/v1/books', async (ctx) => {
  await new Promise((resolve) => {
    db.all('SELECT * FROM books', [], (err, rows) => {
      if (err) {
        ctx.status = 500;
        ctx.body = { error: err.message };
      } else {
        ctx.body = rows;
      }
      resolve();
    });
  });
});

router.get('/api/v1/books/:id', async (ctx) => {
  const id = ctx.params.id;
  await new Promise((resolve) => {
    db.get('SELECT * FROM books WHERE id = ?', [id], (err, row) => {
      if (err) {
        ctx.status = 500;
        ctx.body = { error: err.message };
      } else if (!row) {
        ctx.status = 404;
        ctx.body = { error: 'Book not found' };
      } else {
        ctx.body = row;
      }
      resolve();
    });
  });
});

router.post('/api/v1/books', async (ctx) => {
  const { title, author, published_year } = ctx.request.body;
  if (!title || !author) {
    ctx.status = 400;
    ctx.body = { error: 'Title and author required' };
    return;
  }
  await new Promise((resolve) => {
    db.run('INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)', [title, author, published_year], function (err) {
      if (err) {
        ctx.status = 500;
        ctx.body = { error: err.message };
      } else {
        ctx.status = 201;
        ctx.body = { id: this.lastID, title, author, published_year };
      }
      resolve();
    });
  });
});

router.put('/api/v1/books/:id', async (ctx) => {
  const id = ctx.params.id;
  const { title, author, published_year } = ctx.request.body;
  await new Promise((resolve) => {
    db.run('UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?', [title, author, published_year, id], function (err) {
      if (err) {
        ctx.status = 500;
        ctx.body = { error: err.message };
      } else if (this.changes === 0) {
        ctx.status = 404;
        ctx.body = { error: 'Book not found' };
      } else {
        ctx.body = { id, title, author, published_year };
      }
      resolve();
    });
  });
});

router.delete('/api/v1/books/:id', async (ctx) => {
  const id = ctx.params.id;
  await new Promise((resolve) => {
    db.run('DELETE FROM books WHERE id = ?', [id], function (err) {
      if (err) {
        ctx.status = 500;
        ctx.body = { error: err.message };
      } else if (this.changes === 0) {
        ctx.status = 404;
        ctx.body = { error: 'Book not found' };
      } else {
        ctx.status = 204;
        ctx.body = '';
      }
      resolve();
    });
  });
});

app.use(router.routes()).use(router.allowedMethods());

app.listen(PORT, () => {
  console.log(`Bookstore API (Koa) listening at http://localhost:${PORT}`);
});
