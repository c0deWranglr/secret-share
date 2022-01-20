import { Container, Form, Button } from "react-bootstrap";

export default function StepForm(props) {
    return (
        <Container hidden={props.hidden} className="align-self-center">
            <Form className="pb-3" onSubmit={e => e.preventDefault()}>
                {props.children}
                <Button className="mt-3" variant="outline-secondary" hidden={!props.canClear} onClick={() => props.clearState()}>Reset</Button>
            </Form>
        </Container>
    );
}