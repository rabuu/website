const GHUSER = "rabuu";

class ContentBlock extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    this.classList.add("content-block");
    this.style.display = "block";

    if (this.hasAttribute("desc")) {
      let description = document.createElement("p");
      description.classList.add("content-block-description");
      description.innerHTML = this.getAttribute("desc");
      this.prepend(description);
    }

    if (this.hasAttribute("title")) {
      const title = this.getAttribute("title");

      let headline = document.createElement("div");
      headline.classList.add("content-block-headline");

      let header = document.createElement("h2");
      if (this.hasAttribute("href")) {
        const href = this.getAttribute("href");
        const gh = href === "github";

        let img = document.createElement("img");
        img.classList.add("content-block-link-img");
        img.src = `/assets/${gh ? "github-mark.svg" : "link.svg"}`;

        headline.append(img);

        let link = document.createElement("a");
        link.href = gh ? `https://github.com/${GHUSER}/${title}` : href;
        link.textContent = title;
        header.append(link);
      } else {
        header.textContent = title;
      }

      headline.append(header);
      this.prepend(headline);
    }
  }
}

export default ContentBlock;
