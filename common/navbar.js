var width_query = window.matchMedia("(max-width: 600px)");

width_query.addEventListener("change", function() {
    document.getElementById("links").style.display = width_query.matches ? "none" : "block";
})

function toggle_links() {
    var links = document.getElementById("links");
    if (width_query.matches) {
        if (links.style.display == "none") {
            links.style.display = "block";
        } else {
            links.style.display = "none";
        }
    }
}
