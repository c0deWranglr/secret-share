import {Button, Nav} from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';

export default function LinkButton(props) {
    return (
        <LinkContainer to={props.to}>
            <Nav.Link>
                <Button {...props}>
                    {props.children}
                </Button>
            </Nav.Link>
        </LinkContainer>
    );
}