import React from 'react';
import { BrowserRouter, Link, Route, Switch } from 'react-router-dom';
import './App.css';
import SaveSecret from './components/SaveSecret';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <BrowserRouter>
          <nav>
            <ul>
              <li><Link to="/save">Save</Link></li>
            </ul>
          </nav>
          <Switch>
            <Route path="/save">
              <SaveSecret/>
            </Route>
          </Switch>
        </BrowserRouter>
      </header>
    </div> 
  );
}

export default App;
