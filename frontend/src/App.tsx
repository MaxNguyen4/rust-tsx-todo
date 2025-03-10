import { useState, useEffect } from 'react'

import './App.css'

interface Todo {
  id: number;
  user_id: number;
  todo: string;
  category: string;
  deadline: Date;
}

const URL = "http://localhost:3000";


function App() {

  const [todos, setTodos] = useState<Todo[]>([]);


  useEffect(() => {
    const fetchTodos = async () => {

      const response = await fetch(`${URL}/`);
      const todos = (await response.json()) as Todo[];

      setTodos(todos); 
    };
  
    fetchTodos();
  }, [])



  return (
    <>
      <ul>
        {todos.map((todos) => {
          return <li key={todos.id}> {todos.todo} </li>
        })}

      </ul>

    </>
  )
}

export default App
