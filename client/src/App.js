import React from 'react';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import Home from './routes/Home';
import Header from './components/Header';
import SaveSecret from './components/SaveSecret';
import ViewSecret from './components/ViewSecret';

function App() {

  const name = "Secret Share"; //TODO Get from env

  return (
    <>
    <BrowserRouter>
      <Header brand={name} />
      <Switch>
        <Route exact path="/" render={(props) => { return <Home {...props} name={name} /> }} />
        <Route path="/save" render={(props) => { return <SaveSecret {...props} /> }} />
        <Route path="/view" render={(props) => { return <ViewSecret {...props} /> }} />
      </Switch>
    </BrowserRouter>
    </>
  );
}

export default App;
