const body = document.body;
const comments = document.getElementById("comments");

const addComment = (content) => {
    const el = document.createElement("li")
    el.innerHTML = content;
    return el;
};

const addComments = () => {
    const total = Math.random() * (10 - 1) + 1;
    const ul = document.createElement("ul")
    comments.appendChild(ul);
    for (i = 0; i < total; i++) {
        const li = addComment("This is generated comment " + i);
        ul.appendChild(li);
    }
};


window.addEventListener("load", (event) => {
    body.style.backgroundColor = "#eee";
    addComments();
});
