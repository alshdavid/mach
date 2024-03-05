import * as fsSync from 'node:fs';
import * as path from 'node:path';

export var TargetType;

(function(TargetType) {
    TargetType["FOLDER"] = "FOLDER";
    TargetType["FILE"] = "FILE";
    TargetType["LINK"] = "LINK";
})(TargetType || (TargetType = {}));

export function crawlDir({ 
  targetPath = '', 
  dontCrawl = [], 
  depth = undefined,
  match = [TargetType.FILE, TargetType.FOLDER, TargetType.LINK]
} = {}) {
    const files = new Map();
    let currentDepth = 1;
    function innerCrawl(currentTargetPath) {
        const contents = fsSync.readdirSync(currentTargetPath);
        for (const target of contents){
            const relTargetPath = path.join(currentTargetPath, target);
            const stat = fsSync.lstatSync(relTargetPath);
            if (stat.isSymbolicLink() && match.includes(TargetType.LINK)) {
                files.set(relTargetPath, "LINK");
                continue;
            }
            if (stat.isFile() && match.includes(TargetType.FILE)) {
                files.set(relTargetPath, "FILE");
                continue;
            }
            if (stat.isDirectory()) {
                if (match.includes(TargetType.FOLDER)) {
                  files.set(relTargetPath, "FOLDER");
                }
                if (dontCrawl.includes(target)) {
                    continue;
                }
                if (depth !== undefined && currentDepth !== undefined && currentDepth >= depth) {
                    continue;
                }
                currentDepth++;
                innerCrawl(relTargetPath);
                currentDepth--;
                continue;
            }
        }
    }
    innerCrawl(targetPath);
    return files;
}
