import init, { greet_me, eta_runner } from "./eta.js";

const $in = document.getElementById("in");
const $out = document.getElementById("out");

function println(s) {
$out.textContent += s + "\n";
}

function run() {
const src = $in.value;
if (!src.trim()) return;
println("> " + src);
try {
    println(eta_runner(src));
} catch (e) {
    println("!! Error: " + (e?.message ?? String(e)));
}
$in.value = "";
$in.focus();
}

document.getElementById("run").onclick = run;
document.getElementById("clear").onclick = () => { $out.textContent = ""; $in.focus(); };

$in.addEventListener("keydown", (ev) => {
if (ev.key === "Enter" && !ev.shiftKey) {
    ev.preventDefault();
    run();
}
});

await init();

println("READY.");
$in.focus();
