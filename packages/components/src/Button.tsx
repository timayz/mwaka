import { Component } from "solid-js";
import { useCount } from "mwaka/icons";

const Button: Component = () => {
  const [count, { setCount }] = useCount();

  return <button onClick={() => setCount(count() + 1)}>Click: {count()}</button>;
};

export default Button;
