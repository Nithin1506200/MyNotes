const coins = document.querySelector(".coin").children;

window.addEventListener("mousemove", (e) => {
  const w = window.innerWidth;
  const h = window.innerHeight;
  const [x, y] = [e.x / w - 0.5, e.y / h - 0.5];
  for (let i of coins) {
    i.style.transform = `rotateX(${y * 40}deg) rotateY(${x * 60}deg)`;
  }
});
