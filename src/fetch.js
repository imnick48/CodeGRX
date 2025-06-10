import OpenAI from 'openai';
import { InputSection } from './InpSection.js';
import { initDatabase, createTable, insertData, getById } from './db.js';
import fs from 'fs';
import { SysPrompt } from './SysPrompt.js';

let API_KEY = '';
let db;

async function initializeAPIKey() {
  const dbPath = './.CodeGRX/ProjectGX.sqlite';
  if (!fs.existsSync('.CodeGRX')) {
    fs.mkdirSync('.CodeGRX', { recursive: true });
  }
  db = await initDatabase(dbPath);
  await createTable(db, 'APIKEY', `
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        Keys TEXT NOT NULL UNIQUE
  `);
  const existingKey = await getById(db, 'APIKEY', 1);
  if (existingKey) {
    API_KEY = existingKey.Keys;
  } else {
    API_KEY = await InputSection("Enter your OpenRouter API Key: ");
    await insertData(db, 'APIKEY', {
      Keys: API_KEY
    });
  }

  return API_KEY;
}
const apiKey = await initializeAPIKey();

const openai = new OpenAI({
  baseURL: 'https://openrouter.ai/api/v1',
  apiKey: apiKey,
  defaultHeaders: {
    'X-Title': 'CodeGRX Dev',
    'HTTP-Referer': 'https://sagnickportfolio48.vercel.app/',
    'X-Title': 'CodeGRX'
  },
});

async function AICon(prompt) {
  try {
    const completion = await openai.chat.completions.create({
      model: 'mistralai/mistral-7b-instruct:free',
      temperature: 0.2, 
      response_format: { type: "json_object" }, 
      messages: [
        {
          role: 'system',
          content: SysPrompt
        },
        {
          role: 'user',
          content: prompt,
        },
      ],
    });
    return completion.choices[0].message.content;
  } catch (error) {
    console.error('API Error:', error);
    throw new Error(`AI request failed: ${error.message}`);
  }
}

export { AICon };