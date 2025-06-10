import fs from 'fs';
import path from 'path';


/** 
 * reads the directory structure starting from dirPath
 * @param {string} dirPath - the path to the directory to read
 * @returns {Object} - an object containing the directory structure
*/
function read_directory_structure(dirPath = process.cwd()) {
    function walk(dir) {
        const entries = fs.readdirSync(dir, { withFileTypes: true });
        const result = {
            path: dir,
            folders: [],
            files: []
        };
        entries.forEach(entry => {
            const fullPath = path.join(dir, entry.name);
            if (entry.isDirectory()) {
                if (entry.name != 'node_modules') {
                    result.folders.push(walk(fullPath));
                }
                else {
                    result.folders.push(fullPath);
                }
            }
            else {
                result.files.push(fullPath);
            }

        })
        return result;
    }
    return walk(dirPath);
}
// Update isValidJSON to actually validate
function isValidJSON(str) {
    try {
        JSON.parse(str);
        return true;
    } catch {
        return false;
    }
}

// Enhance json_decoder
function json_decoder(str) {
    try {
        // Try direct parse first
        try {
            return JSON.parse(str);
        } catch {
            // Try extracting JSON from markdown
            const jsonMatch = str.match(/```(?:json)?\n([\s\S]+?)\n```/);
            if (jsonMatch) return JSON.parse(jsonMatch[1]);
            throw new Error("No valid JSON found");
        }
    } catch (e) {
        console.error("Failed to parse:", str);
        throw new Error(`JSON parse error: ${e.message}`);
    }
}

function create_folders(folders) {
    folders.forEach(folder => {
        const fullPath = path.resolve(folder);
        if (!fs.existsSync(fullPath)) {
            fs.mkdirSync(fullPath, { recursive: true });
        }
    });
}

function create_files(files) {
    files.forEach(file => {
        const fullPath = path.resolve(file);
        const dir = path.dirname(fullPath);
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
        }
        if (!fs.existsSync(fullPath)) {
            fs.writeFileSync(fullPath, '', 'utf8')
        }
    })

}


function add_lines(lines, lineNo, content) {
    while (lines.length < lineNo) {
        lines.push("");
    }
    lines.splice(lineNo, 0, content);
}
function delete_line(lines, lineNo) {
    lines.splice(lineNo, 1);
}

function modify_line(lines, lineNo, content) {
    lines[lineNo] = content;
}


function fileToLinesArray(filePath) {
    try {
        const content = fs.readFileSync(filePath, 'utf8');
        return content.split(/\r?\n/);
    } catch (err) {
        console.error(`Error reading file: ${filePath}`, err);
        return [];
    }
}

function writeLinesToFile(filePath, linesArray) {
    try {
        const content = linesArray.join('\n');
        fs.writeFileSync(filePath, content, 'utf8');
        console.log(`File written: ${filePath}`);
    } catch (err) {
        console.error(`Error writing file: ${filePath}`, err);
    }
}




export { isValidJSON, json_decoder, create_folders, create_files, read_directory_structure, add_lines, delete_line, modify_line, fileToLinesArray, writeLinesToFile };