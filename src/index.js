import { AICon } from "./fetch.js";
import {
    isValidJSON,
    json_decoder,
    create_folders,
    create_files,
    add_lines,
    delete_line,
    modify_line,
    fileToLinesArray,
    writeLinesToFile
} from "./utills.js";
import path from "path";
import { InputSection } from "./InpSection.js";
import { initDatabase, createTable,insertData, getAll } from "./db.js";
import fs from 'fs';

let db;
if (!fs.existsSync('./.CodeGRX/ProjectGX.sqlite')) {
    if (!fs.existsSync('.CodeGRX')) {
        fs.mkdirSync('.CodeGRX', { recursive: true });
    }
    db = initDatabase('./.CodeGRX/ProjectGX.sqlite');
    createTable(db, 'PROMPTS', `
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    Prompts TEXT NOT NULL
  `);
}

else {
    db = initDatabase('./.CodeGRX/ProjectGX.sqlite');
    createTable(db, 'PROMPTS', `
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    Prompts TEXT NOT NULL
    `);
}


async function main(prompt) {
    const prompts = getAll(db, 'PROMPTS');
    prompt = prompt + "Prompts By user previously:\n" + prompts.map(p => `\nPrevious Prompt: ${p.Prompts}`).join('');
    const data = await AICon(prompt);
    if (!isValidJSON(data) || isValidJSON(data)==null) {
        console.log(data);;
        return;
    }
    const j = json_decoder(data);
    console.log(j);
    create_folders(j.folder_paths);
    create_files(j.files_path);
    for (const fileChange of j.file_contents_and_changes) {
        const filePath = path.resolve(fileChange.file_path);
        let lines = fileToLinesArray(filePath);

        for (const change of fileChange.changes) {
            if (change.action === "add") {
                add_lines(lines, change.line, change.content);
            } else if (change.action === "modify") {
                modify_line(lines, change.line, change.content);
            } else if (change.action === "delete") {
                delete_line(lines, change.line);
            }
        }

        writeLinesToFile(filePath, lines);
        if (fileChange.read_files != null && fileChange.read_files.length > 0) {
            const contents = fileChange.read_files.map(file => {
                const readFilePath = path.resolve(file);
                const readLines = fileToLinesArray(readFilePath);
                return `Contents of ${readFilePath}: \n${readLines.join("\n")}`;
            }).join("\n\n");
            console.log(`Requested files to read: ${fileChange.read_files.join(", ")}`);
            const newPrompt = `${prompt}
                Please take into account the following file contents before proceeding:
        ${contents}`;
            fileChange.read_files = [];
            await main(newPrompt);
        }
    }
}

const UserPrompt = await InputSection("Give the Prompt")
if (!UserPrompt) {
    console.error("No prompt provided. Exiting.");
    process.exit(1);
}
insertData(db, 'PROMPTS', {
    Prompts: UserPrompt
});
main(UserPrompt);
