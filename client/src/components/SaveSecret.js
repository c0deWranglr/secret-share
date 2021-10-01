import React from 'react';
import { Link } from 'react-router-dom';
import { Form, Container, Button, FloatingLabel, Toast, Row, Col } from 'react-bootstrap';
import { storeSecret } from '../lib/client';
import { encrypt } from '../lib/cryptography';

class SaveSecret extends React.Component {
    constructor(props) {
        super(props);
        this.state = { password: '', token: '', submitted: false, error: null, redemptionKey: null, expirationType: null, expirationValue: '' };

        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.componentDidMount = this.componentDidMount.bind(this);
    }

    componentDidMount() {
        this.setState({ password: '', token: '', submitted: false, error: null, redemptionKey: null, expirationType: null, expirationValue: '' });
    }

    handleChange(event) {
        let target = event.target;
        var value = target.value;
        if (target.name === "expirationType") {
            //Expiration is a radial button that needs to set the id of expirationType
            value = target.id;
        }

        this.setState({ [target.name]: value });
    }

    handleSubmit(event) {
        this.setState( { error: null });

        let data = encrypt(this.state.token, this.state.password);
        let ttl = null;
        let attempts = null; 
        if (this.state.expirationType === "ttl") { 
            ttl = this.state.expirationValue;
        } 
        else if (this.state.expirationType === "attempts") { 
            attempts = this.state.expirationValue; 
        }
        storeSecret(data, ttl, attempts).then(key => {
            if (key !== undefined) {
                this.setState( { redemptionKey: key, submitted: true });
            } else {
                this.setState( { error: "⚠️ Server unresponsive, please try again later." });
            }
        });
        event.preventDefault();
    }

    render() {
        return (
            <Container>
                <h2 className="mt-3" style={{marginLeft:"20px"}}>Save Secret</h2>
                <p className="mt-3" style={{fontSize:"small", marginLeft:"20px", marginRight:"30px"}}>
                    Easily save a password or secret that you'd like to share. A token (or passphrase) is required to ensure safe encryption. The server storing the
                    password has no ability to know what the password is. This is because the password is encrypted using a hash generated by the redemption token. Only
                    encrypted password data is sent to and from the server. The server will generate and return a key to use to access the encrypted password data. 
                </p>
                <Container hidden={this.state.redemptionKey!=null}>
                    <Form onSubmit={this.handleSubmit}>
                        <FloatingLabel className="mb-3 mt-3" label="Password">
                            <Form.Control required name="password" type="password" placeholder="" onChange={this.handleChange} value={this.state.password}  />
                            <Form.Text>
                                The password (or secret) to share.
                            </Form.Text>
                        </FloatingLabel>
                        <FloatingLabel className="mb-3" label="Redepmtion Token">
                            <Form.Control required name="token" type="password" placeholder="" onChange={this.handleChange} value={this.state.token} />
                            <Form.Text>
                                A token or passphrase used to allow viewing the password.
                            </Form.Text>
                        </FloatingLabel>
                        <Row key="inline-radio">
                            <Col>
                            <FloatingLabel label="Expiration">
                                <Form.Control required name="expirationValue" type="number" placeholder="" onChange={this.handleChange} value={this.state.expirationValue} />
                                <Form.Text>
                                    How long the password will be available to share. 
                                    <br/>TTL - # of minutes to store password. 
                                    <br/>Attempts - # of times a password is allowed to be accessed.
                                </Form.Text>
                            </FloatingLabel>
                            </Col>
                            <Col>
                                <Form.Check required label="TTL" name="expirationType" type="radio" id="ttl" onChange={this.handleChange} checked={this.state.expirationType == "ttl"} />
                                <Form.Check required label="Attempts" name="expirationType" type="radio" id="attempts" onChange={this.handleChange} checked={this.state.expirationType == "attempts"} />
                            </Col>
                        </Row>
                        <Row>
                            <Col md="auto">
                                <Button className="mt-1" variant="primary" type="submit">Submit</Button>
                            </Col>
                            <Col>
                                <Toast show={this.state.error != null} onClose={() => this.setState({ error: null })} autohide delay={10000}>
                                    <Toast.Body>{this.state.error}</Toast.Body>
                                </Toast>
                            </Col>
                        </Row>
                    </Form>
                </Container>
                <Container hidden={this.state.redemptionKey==null}>
                    <Row className="justify-content-md-center">
                        <Col md="auto">
                            <label>
                                Redemption Key
                            </label>
                        </Col>
                    </Row>
                    <Row className="justify-content-md-center mt-3 mb-3">
                        <Col md="auto">
                            <Link to={"/view?key="+this.state.redemptionKey}>
                                <h2>{this.state.redemptionKey}</h2>
                            </Link>
                        </Col>
                    </Row>
                    <Row className="justify-content-md-center">
                        <Col md="auto">
                            <Button onClick={this.componentDidMount}>Save Another</Button>
                        </Col>
                    </Row>
                </Container>
            </Container>
        );
    }
}

export default SaveSecret;