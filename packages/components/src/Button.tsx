import { Component } from "solid-js";
import { useCount } from "mwaka/aria";

const Button: Component = () => {
  const [count, { setCount }] = useCount();
  return <button onClick={() => setCount(count() + 1)}>{count()}</button>;
};

export default Button;
