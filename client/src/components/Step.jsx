import { Form, Row, Col, Button } from "react-bootstrap";

export default function Step(props) {
    return (
        <Row hidden={props.hidden} className="mt-3">
            <Form.Label sm="12" md="3" column className="text-start">{props.label}</Form.Label>
            <Col xs="8" md="7">
                {props.children}
            </Col>
            <Col className={"d-flex justify-content-end " + (props.buttonPlacement === "bottom" ? "align-items-end" : "align-items-start")}>
                <Button type={props.showButton ? "submit" : ""} variant="primary" hidden={!props.showButton} disabled={!props.showButton} onClick={props.onButtonClick}>{props.buttonText}</Button>
            </Col>
        </Row>
    );
}