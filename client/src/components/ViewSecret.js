import React from 'react';
import { Form, Container, Button, FloatingLabel, Toast, Row, Col } from 'react-bootstrap';
import { getSecret } from '../lib/client';
import { decrypt } from '../lib/cryptography';

class ViewSecret extends React.Component {
    constructor(props) {
        super(props);

        console.log("Location: "+JSON.stringify(props.location));

        this.loadSecret = this.loadSecret.bind(this);
        this.componentDidMount = this.componentDidMount.bind(this);
        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.extractKey = this.extractKey.bind(this);

        let key = this.extractKey(props.location);
        this.state = { needToken: true, lastKey: key, redemptionKey: key, secret: '', token: '', decrypted: '' };
    }

    extractKey(location) {
        let params = location.search;
        let start = params.indexOf("key=")+4;
        var end = params.indexOf("&", start);
        if (end < 0) { end = params.length; }
        return params.substring(start, end);
    }
    
    componentDidMount() {
        this.setState({ needToken: true, secret: '', token: '', decrypted: '' }, () => {
            this.loadSecret();
        });
    }

    loadSecret() {
        if (this.state.redemptionKey !== '') {
            console.log("Loading secret for key: "+this.state.redemptionKey);

            this.props.history.push(document.location.pathname+"?key="+this.state.redemptionKey);
            
            getSecret(this.state.redemptionKey).then(secret => {
                console.log("Secret: "+secret);
                this.setState({ secret: secret === undefined || secret === null ? '' : secret });
            });
        } else {
            this.setState({ secret: '' })
        }
        
        this.setState({ lastKey: this.state.redemptionKey, needToken: true, token: '', decrypypted: '' });
    }

    handleChange(event) {
        this.setState({ [event.target.name]: event.target.value });
    }

    handleSubmit(event) {
        let decrypted = decrypt(this.state.token, this.state.secret);
        console.log("Token: "+this.state.token+"; Secret: "+this.state.secret+"; Decrypted: "+decrypted);
        this.setState({ needToken: decrypted === '', decrypted: decrypted });
        event.preventDefault();
    }

    render() {
        return (
            <Container>
                <h2 className="mt-3" style={{marginLeft:"20px"}}>View Secret</h2>
                <p className="mt-3" style={{fontSize:"small", marginLeft:"20px", marginRight:"30px"}}>
                    View the secret. 
                </p>
                <Form onSubmit={(e) => {this.loadSecret(); e.preventDefault(); }}>
                    <Row>
                        <Col>
                            <FloatingLabel label="Key">
                                <Form.Control required name="redemptionKey" type="text" onChange={this.handleChange} value={this.state.redemptionKey} />
                                <Form.Text>
                                    The key (4+ character code) provided when the secret was saved.
                                </Form.Text>
                            </FloatingLabel>
                        </Col>
                        <Col>
                            <Button type="submit" disabled={this.state.lastKey == this.state.redemptionKey}>Fetch</Button>
                        </Col>
                    </Row>
                </Form>
                <Form onSubmit={this.handleSubmit}>
                    <Row className="justify-content-md-center mt-3 mb-3">
                        <Col>
                            <FloatingLabel label="Token">
                                <Form.Control disabled={this.state.secret === '' || !this.state.needToken} required name="token" type="password" onChange={this.handleChange} value={this.state.token} />
                                <Form.Text>
                                    The token used at the time of saving to secure the secret.
                                </Form.Text>
                            </FloatingLabel>
                        </Col>
                        <Col>
                            <Button disabled={this.state.secret === '' || !this.state.needToken} type="submit">View</Button>
                        </Col>
                    </Row>
                    <Row className="justify-content-md-center">
                        <Col>
                            <Form.Control plaintext readOnly type={this.state.needToken ? "password" : "text"} value={this.state.needToken ? this.state.secret : this.state.decrypted} />
                        </Col>
                    </Row>
                </Form>
                {/* <Container hidden={this.state.secret === undefined || this.state.secret === null}>
                    <div hidden={!this.state.needToken}>
                    <form onSubmit={this.handleSubmit}>
                            <label>Token:</label>
                            <input name="token" type="password" onChange={this.handleChange}/>
                            <input type="submit" value="Submit" />
                        </form>
                    </div>
                    <div hidden={this.state.needToken}>
                        <label>Secret: { this.state.decrypted }</label>
                    </div>
                </Container> */}
            </Container>
        );
    }
}

export default ViewSecret;