import { Maze } from "../pkg";
import { jsPDF } from "jspdf";
import { font } from "./courier-new-normal.js"

const printButton = document.getElementById("print-btn");
const easyButton = document.getElementById("easy-btn");
const mediumButton = document.getElementById("medium-btn");
const hardButton = document.getElementById("hard-btn");
const veryHardButton = document.getElementById("very-hard-btn");
const veryVeryHardButton = document.getElementById("very-very-hard-btn");
const fontSizeInput = document.getElementById("font-size");
const seedInput = document.getElementById("seed");
const widthInput = document.getElementById("width");
const heightInput = document.getElementById("height");

const parseInput = (inputValue, defaultValue) => {
  let value = parseInt(inputValue, 10);
  if (isNaN(value) || value < 1) {
    value = defaultValue;
  }
  return value;
}

const getInputs = () => {
  const seed = parseInput(seedInput.value, 1);
  const width = parseInput(widthInput.value, 10);
  const height = parseInput(heightInput.value, 10);
  return { width: width, height: height, seed: seed };
}

const buildMaze = (width, height, seed) => {
  const maze = Maze.new(width, height);
  maze.generate(BigInt(seed));
  return maze;
}

const renderToPdf = (maze, fontSize) => {
  const doc = new jsPDF();
  doc.addFileToVFS('courier-new-normal.ttf', font);
  doc.addFont('courier-new-normal.ttf', 'courier-new', 'normal');
  doc.setFont("courier-new");
  doc.setFontSize(fontSize);
  doc.setLineHeightFactor(1);
  const text = maze.render();
  const xOffset = doc.internal.pageSize.width / 2;
  doc.text(text, xOffset, 10, null, null, "center");
  doc.save(`maze-${maze.get_width()}x${maze.get_height()}.pdf`);
};

printButton.addEventListener("click", () => {
  const { width, height, seed } = getInputs();
  const maze = buildMaze(width, height, seed);
  const fontSize = parseInput(fontSizeInput.value, 10);
  renderToPdf(maze, fontSize);
});

easyButton.addEventListener("click", (e) => {
  e.preventDefault();
  widthInput.value = 8;
  heightInput.value = 8;
  fontSizeInput.value = 25;
});

mediumButton.addEventListener("click", (e) => {
  e.preventDefault();
  widthInput.value = 30;
  heightInput.value = 30;
  fontSizeInput.value = 6;
});

hardButton.addEventListener("click", (e) => {
  e.preventDefault();
  widthInput.value = 71;
  heightInput.value = 130;
  fontSizeInput.value = 3;
});

veryHardButton.addEventListener("click", (e) => {
  e.preventDefault();
  widthInput.value = 116;
  heightInput.value = 199;
  fontSizeInput.value = 2;
});

veryVeryHardButton.addEventListener("click", (e) => {
  e.preventDefault();
  widthInput.value = 232;
  heightInput.value = 400;
  fontSizeInput.value = 1;
});
