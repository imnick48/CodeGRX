import Database from "better-sqlite3";

function initDatabase(filePath = './database.sqlite') {
  return new Database(filePath);
}
function createTable(db, tableName, schema) {
  const query = `CREATE TABLE IF NOT EXISTS ${tableName} (${schema})`;
  db.exec(query);
  console.log(`Table ${tableName} created/verified`);
}
function insertData(db, tableName, data) {
  const columns = Object.keys(data).join(', ');
  const placeholders = Object.keys(data).fill('?').join(', ');
  const values = Object.values(data);
  
  const stmt = db.prepare(
    `INSERT INTO ${tableName} (${columns}) VALUES (${placeholders})`
  );
  
  const result = stmt.run(...values);
  console.log(`Inserted row with ID: ${result.lastInsertRowid}`);
  return result;
}

function getAll(db, tableName) {
  const stmt = db.prepare(`SELECT * FROM ${tableName}`);
  return stmt.all();
}
function getById(db, tableName, id) {
  const stmt = db.prepare(`SELECT * FROM ${tableName} WHERE id = ?`);
  return stmt.get(id);
}

function deleteData(db, tableName, id) {
  const stmt = db.prepare(`DELETE FROM ${tableName} WHERE id = ?`);
  const result = stmt.run(id);
  console.log(`Deleted ${result.changes} row(s)`);
  return result;
}


export {
  initDatabase,
  createTable,
  insertData,
  getAll,
  getById,
  deleteData
};
// // Close database
// function closeDatabase(db) {
//   db.close();
//   console.log('Database connection closed');
// }

// // Example usage
// const db = initDatabase();

// // Create users table
// createTable(db, 'users', `
//   id INTEGER PRIMARY KEY AUTOINCREMENT,
//   name TEXT NOT NULL,
//   email TEXT UNIQUE NOT NULL,
//   age INTEGER
// `);

// // Insert a user
// insertData(db, 'users', {
//   name: 'Alice',
//   email: 'alice@example.com',
//   age: 30
// });

// // Get all users
// const allUsers = getAll(db, 'users');
// console.log('All users:', allUsers);

// // Close when done
// closeDatabase(db);