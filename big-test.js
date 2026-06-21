const res = await fetch("https://api.github.com/repos/ewwii-sh/ewwii");
console.log(await res.json())
console.log(await res.text())
