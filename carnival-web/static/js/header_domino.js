<script>
// cba. Let's us avoid Javascript's async execution from fucking us.

// ^.^ gets replaced with header text passed into animations::animate_header(&str) in `rendering/components.rs`.
// >_<
document.getElementById('animated-header').textContent = document.getElementById('animated-header').textContent = "^.^"; 
document.getElementById('animated-header').innerHTML = document.getElementById('animated-header').textContent.replace(/\S/g, '<span class=\"letter\">$&</span>');
anime.timeline({loop: false})
  .add({
    targets: '#animated-header .letter',
    translateX: [40, 0],
    translateZ: 0,
    opacity: [0, 1],
    easing: 'easeOutExpo',
    duration: 700,
    delay: (el, i) => 500 + 30 * i
  }).add({
    targets: '#animated-header',
    backgroundSize: '100%',
    duration: 800,
    easing: 'easeOutExpo'
  });
</script>
