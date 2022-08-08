import { Maze } from "../pkg";

const heigth = 40;
const width = 80;
const maze = Maze.new(width, heigth);

const pre = document.getElementById("pre");
const button = document.getElementById("generate-btn");

button.addEventListener("click", event => {
  maze.generate(BigInt(11));
  pre.textContent = maze.render();
});
