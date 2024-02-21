import * as wasm from "wasm-random-walk";

let geneList = [];
let network = [];
document.getElementById("genelist").addEventListener("change", getGeneList);
document.getElementById("network").addEventListener("change", getNetwork);
document
  .getElementById("submit")
  .addEventListener("click", () => start(network, geneList));
document.getElementById("example").addEventListener("click", start_example);
function start_example() {
  fetch("./data/funmap.tsv")
    .then((response) => response.text())
    .then((network_text) => {
      fetch("./data/genelist.txt")
        .then((response) => response.text())
        .then((gene_list_text) => {
          let ex_geneList = gene_list_text.split("\n");
          let ex_network = network_text.split("\n");
          console.log(ex_geneList);
          start(ex_network, ex_geneList);
        });
    });
}

function check_button() {
  if (geneList.length > 0 && network.length > 0) {
    document.getElementById("submit").disabled = false;
  } else {
    document.getElementById("submit").disabled = true;
  }
}

function getGeneList() {
  let file = document.getElementById("genelist").files[0];
  let reader = new FileReader();
  reader.readAsText(file);
  reader.onload = function () {
    geneList = reader.result.split("\n");
    check_button();
  };
}
function getNetwork() {
  let file = document.getElementById("network").files[0];
  let reader = new FileReader();
  reader.readAsText(file);
  reader.onload = function () {
    network = reader.result.split("\n");
    check_button();
  };
}

function start(input_network, input_geneList) {
  // array of strings
  let start_time = performance.now();
  let res = wasm.get_neighborhood(input_network, input_geneList, 49);
  let res_area = document.getElementById("results");
  res_area.style.display = "block";
  let output = document.getElementById("output");
  output.innerHTML = res.join(", ");
  let end_time = performance.now();
  let time = end_time - start_time;
  let time_area = document.getElementById("time_area");
  time_area.innerHTML = "Time: " + time + " ms";
}
