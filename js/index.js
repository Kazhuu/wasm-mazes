import { Maze } from "../pkg";

const pre = document.getElementById("pre");
const button = document.getElementById("generate-btn");
const seed_input = document.getElementById("seed");
const width_input = document.getElementById("width");
const heigth_input = document.getElementById("height");

const parse_input = (input_value, default_value) => {
  let value = parseInt(input_value, 10);
  if (isNaN(value) || value < 1) {
    value = default_value;
  }
  return value;
}

button.addEventListener("click", event => {
  const seed = parse_input(seed_input.value, 1);
  const width = parse_input(width_input.value, 10);
  const height = parse_input(heigth_input.value, 10);
  const maze = Maze.new(width, height);
  maze.generate(BigInt(seed));
  pre.textContent = maze.render();
});
