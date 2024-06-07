import { FormEvent, useState } from "react";
import axios from "axios";

function App() {
  const [message, setMessage] = useState("");
  const [username, setUsername] = useState("");

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault(); // Prevents the default form submission behavior

    // Make the API request only if username is not empty
    if (username.trim() !== "") {
      axios
        .get(`https://127.0.0.1:16600/api/hello/${username}`)
        .then((response) => {
          setMessage(response.data);
        })
        .catch((error) => {
          console.error("Error fetching data:", error);
        });
    }
  };
  return (
    <>
      <h1 className="text-center">Reactix</h1>
      <form onSubmit={handleSubmit} className="flex flex-col text-center">
        <label htmlFor="username">Enter your username</label>
        <input
          className={"bg-rose-400 text-gray-50"}
          name="username"
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />

        <input type="submit"></input>
      </form>
      <p>{message}</p>
    </>
  );
}

export default App;
