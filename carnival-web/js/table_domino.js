// See the comically dogshit way this is being mutated in `rendering/components.rs`
const rows = document.querySelectorAll(".row");

// Function to animate a single row
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

// Loop through the rows and animate them with a delay
rows.forEach((row, index) => {
  animateRow(row, index * 10); // Adjust the delay as needed
});
