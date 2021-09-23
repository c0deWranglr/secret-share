import React from 'react';
import { BrowserRouter, Link, Route, Switch } from 'react-router-dom';
import './App.css';
import SaveSecret from './components/SaveSecret';
import ViewSecret from './components/ViewSecret';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <BrowserRouter>
          <nav>
            <Link to="/save">Save</Link>
            <br/>
            <Link to="/view">View</Link>
          </nav>
          <Switch>
            <Route path="/save">
              <SaveSecret/>
            </Route>
            <Route path="/view" render={(props) => { return <ViewSecret {...props} /> }}>
            </Route>
          </Switch>
        </BrowserRouter>
      </header>
    </div> 
  );
}

export default App;
