import init, { solve } from "./pkg/aoc2021.js";

function generate_solution() {
  let input = document.getElementById("day01Input").value;
  input = input.split("\n").map(s => s.trim()).filter(i => i !== "").map(Number);
  solve(input);
}

init().then(() => {
  fetch("./inputs/day01").then(response => response.text()).then(input => document.getElementById("day01Input").value = input).then(() => {
    let button = document.getElementById("day01Solve");
    button.onclick = generate_solution;
    button.disabled = false;
  });
});
