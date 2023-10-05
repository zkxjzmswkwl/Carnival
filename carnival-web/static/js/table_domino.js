const rows = document.querySelectorAll(".row");

function animateRow(row, delay) {
  anime({
    targets: row,
    opacity: [0, 1],
    translateY: [10, 0],
    easing: "easeOutExpo",
    duration: 800,
    delay: delay,
  });
}

rows.forEach((row, index) => {
  animateRow(row, index * 20);
});
