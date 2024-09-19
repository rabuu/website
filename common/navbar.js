var width_query = window.matchMedia("(max-width: 600px)");

width_query.addEventListener("change", function() {
    document.getElementById("links").style.display = width_query.matches ? "none" : "block";
})

function nav_toggle_links() {
    var links = document.getElementById("links");
    if (width_query.matches) {
        if (links.style.display == "block") {
            links.style.display = "none";
        } else {
            links.style.display = "block";
        }
    }
}
