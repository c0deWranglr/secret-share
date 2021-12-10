import React from 'react';
import { LinkContainer } from 'react-router-bootstrap';
import { Container, Navbar, Nav, Image } from 'react-bootstrap';

export default function Header(props) {
    return (
        <Navbar bg="light" expand="sm">
            <Container>
            <Navbar.Brand href="/">
                <Image src="/logo32.png" />
            </Navbar.Brand>
            <Navbar.Toggle aria-controls="basic-navbar-nav" />
            <Navbar.Collapse id="basic-navbar-nav">
                <Nav className="me-auto" variant="tabs">
                <LinkContainer to="/save" className="ps-3 pe-3"><Nav.Link>Save</Nav.Link></LinkContainer>
                <LinkContainer to="/view" className="ps-3 pe-3"><Nav.Link>View</Nav.Link></LinkContainer>
                </Nav>
                <Nav>
                    <Nav.Link href="https://github.com/c0dewranglr/secret-share"><Image src="/assets/github.png"/></Nav.Link>
                </Nav>
            </Navbar.Collapse>
            </Container>
        </Navbar>
    );
}