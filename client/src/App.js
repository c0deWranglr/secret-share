import React from 'react';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import { LinkContainer } from 'react-router-bootstrap';
import { Container, Navbar, Nav } from 'react-bootstrap';
import SaveSecret from './components/SaveSecret';
import ViewSecret from './components/ViewSecret';

function App() {
  return (
    <>
    <BrowserRouter>
      <Navbar bg="light" expand="lg">
        <Container>
          <Navbar.Brand href="/">Secret-Share</Navbar.Brand>
          <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="me-auto">
              <LinkContainer to="/save"><Nav.Link>Save</Nav.Link></LinkContainer>
              <LinkContainer to="/view"><Nav.Link>View</Nav.Link></LinkContainer>
            </Nav>
          </Navbar.Collapse>
        </Container>
      </Navbar>
        <Switch>
          <Route path="/save">
            <SaveSecret/>
          </Route>
          <Route path="/view" render={(props) => { return <ViewSecret {...props} /> }}>
          </Route>
        </Switch>
    </BrowserRouter>
    </> 
  );
}

export default App;
