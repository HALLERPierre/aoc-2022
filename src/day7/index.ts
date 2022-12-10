import fs from "fs";

type Directory = {
  subdirectories: Directory[];
  parent: Directory | null;
  files: number[];
  size: number;
};

const GET_FILE_REGEX = new RegExp(/(\d+)/);

const getFileSize = (line: string): number => {
  const match = GET_FILE_REGEX.exec(line);
  if (match === null) {
    throw new Error("No file size found");
  }
  return parseInt(match[0]);
};

const parseFile = () => {
  const content = fs
    .readFileSync(__dirname + "/input")
    .toString()
    .split("\n")
    .filter((line) => line.length > 0);

  const rootDirectory: Directory = {
    subdirectories: [],
    parent: null,
    files: [],
    size: 0,
  };

  let currentDirectory = rootDirectory;

  content.forEach((line) => {
    if (line == "$ cd /" || line.startsWith("dir") || line === "$ls") {
      return;
    }
    // change dir
    if (line.startsWith("$ cd")) {
      // return to parent
      if (line === "$ cd ..") {
        if (currentDirectory.parent === null) {
          throw new Error("can't get parent");
        }
        currentDirectory = currentDirectory.parent;
      }
      // Going into child dir
      else {
        const subDirectory = {
          subdirectories: [],
          files: [],
          parent: currentDirectory,
          size: 0,
        };
        currentDirectory.subdirectories.push(subDirectory);
        currentDirectory = subDirectory;
      }
    }
    // it's a file
    if (!line.startsWith("$")) {
      const fileSize = getFileSize(line);
      currentDirectory.files.push(fileSize);
    }
  });
  return rootDirectory;
};

const computeSize = (directory: Directory) => {
  directory.subdirectories.forEach((subdir) => {
    computeSize(subdir);
  });

  const childrenSize = directory.subdirectories.reduce(
    (sum, subdir) => subdir.size + sum,
    0
  );
  const filesSize = directory.files.reduce(
    (sum, fileSize) => fileSize + sum,
    0
  );
  directory.size = childrenSize + filesSize;
};

const flattenDirectories = (directory: Directory): number[] => {
  const dirs: number[] = [directory.size];
  let dirsRemaining = directory.subdirectories;
  while (dirsRemaining.length > 0) {
    const currentDir = dirsRemaining.pop();
    if (currentDir === undefined) {
      throw new Error("should not be undefined");
    }
    dirsRemaining = [...dirsRemaining, ...currentDir.subdirectories];
    dirs.push(currentDir.size);
  }

  return dirs;
};

const main1 = () => {
  const rootDirectory = parseFile();
  computeSize(rootDirectory);

  const directories = flattenDirectories(rootDirectory);

  const directoriesWithMaxSize = directories.filter((size) => size <= 100000);

  const sumDirectoriesSize = directoriesWithMaxSize.reduce(
    (total, size) => total + size,
    0
  );
  console.log(`1_ sum of dirs with size <= 100000 ${sumDirectoriesSize}`);
};

const TOTAL_SPACE = 70000000;
const FREE_SPACE = 30000000;

const main2 = () => {
  const rootDirectory = parseFile();
  computeSize(rootDirectory);

  const freeSpace = TOTAL_SPACE - rootDirectory.size;
  const neededSpace = FREE_SPACE - freeSpace;

  const directories = flattenDirectories(rootDirectory);
  const sortedDirectories = directories.sort((a, b) => a - b);

  const smallestDirToDelete = sortedDirectories.find(
    (size) => size >= neededSpace
  );

  console.log(`2_ smallest dir to delete is ${smallestDirToDelete}`);
};

main1();
main2();
