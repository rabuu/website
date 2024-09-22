class ProjectBlock extends HTMLElement {
  // constructor() {
  //   super();
  // }

  connectedCallback() {
    this.classList.add("project");
    this.style.display = "block";

    const name = this.getAttribute("name") || "";
    const isGithub = !this.hasAttribute("href");

    let headline = document.createElement("div");
    headline.classList.add("project-headline");

    let linkImg = document.createElement("img");
    linkImg.src = isGithub ? "/assets/github-mark.svg" : "/assets/link.png";
    linkImg.alt = isGithub ? "GitHub link to project" : "Link to project";
    headline.append(linkImg);

    let header = document.createElement("h2");
    let link = document.createElement("a");
    link.href = this.getAttribute("href") || `https://github.com/rabuu/${name}`;
    link.textContent = name;
    header.append(link);
    headline.append(header);

    this.append(headline);

    if (this.hasAttribute("descr")) {
      let description = document.createElement("p");
      description.classList.add("project-description");
      description.innerHTML = this.getAttribute("descr");
      this.append(description);
    }
  }
}

customElements.define("project-block", ProjectBlock);
