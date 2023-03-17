import "./App.css";

async function postData(task) {
  // Default options are marked with *
  const response = await fetch("http://127.0.0.1:8000/task", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(task),
  });
  return response.json();
}

function App() {
  const onClick = () => {
    for (let index = 0; index < 200; index++) {
      postData({ title: "test", description: "Descriptions is long" });
    }
  };
  return (
    <div className="App">
      <div>
        <button onClick={onClick}>Start the load</button>
      </div>
    </div>
  );
}

export default App;
