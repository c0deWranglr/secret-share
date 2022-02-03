import React from 'react';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import Home from './routes/Home';
import View from './routes/View';
import Header from './components/Header';
import SaveSecret from './components/SaveSecret';

function App() {

  const name = window["config"].siteName;
  document.title = name;

  return (
    <>
    <BrowserRouter>
      <Header brand={name} />
      <Switch>
        <Route exact path="/" render={(props) => { return <Home {...props} name={name} /> }} />
        <Route path="/save" render={(props) => { return <SaveSecret {...props} /> }} />
        <Route path="/view" render={(props) => { return <View {...props} /> }} />
      </Switch>
    </BrowserRouter>
    </>
  );
}

export default App;
