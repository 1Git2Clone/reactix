import React from "react";

function MyButton(): Element {
  return <button>I'm a button</button>;
}

export default function App(): Element {
  return (
    <div>
      <h1>Welcome to my app</h1>
      <MyButton />
    </div>
  );
}
