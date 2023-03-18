const body = document.body;
const comments = document.getElementById("comments");

const getComment = async (content, callback) => {
    const options = {
        headers: {
            "Accept": "application/json",
        },
    }
    const res = await fetch("https://icanhazdadjoke.com/", options)
    if (res.ok) {
        const data = await res.json();
        return Promise.resolve(data);
    }

    return Promise.reject("Failed to load VIP comment");
};

const loadComments = () => {
    const total = Math.random() * (10 - 1) + 1;
    const ul = document.createElement("ul")
    comments.appendChild(ul);
    for (i = 0; i < total; i++) {
        getComment()
        .then(data => {
            const li = document.createElement("li");
            li.innerHTML = data.joke;
            ul.appendChild(li);
        })
        .catch(err => {
            console.error("err", err);
        });
    }
};

window.addEventListener("load", (event) => {
    body.style.backgroundColor = "#ffe";
    loadComments();
});
