import React from 'react';
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

        this.state = { needToken: true, redemptionKey: this.extractKey(props.location), secret: null, token: '', decrypted: '' };
    }

    extractKey(location) {
        let params = location.search;
        let start = params.indexOf("key=")+4;
        var end = params.indexOf("&", start);
        if (end < 0) { end = params.length; }
        return params.substring(start, end);
    }
    
    componentDidMount() {
        this.setState({ needToken: true, secret: null, token: '', decrypted: '' }, () => {
            this.loadSecret();
        });
    }

    loadSecret() {
        if (this.state.redemptionKey !== '') {
            console.log("Loading secret for key: "+this.state.redemptionKey);
            getSecret(this.state.redemptionKey).then(secret => {
                console.log("Secret: "+secret);
                this.setState({ secret: secret });
            });
        }
    }

    handleChange(event) {
        this.setState({ [event.target.name]: event.target.value });
    }

    handleSubmit(event) {
        let decrypted = decrypt(this.state.token, this.state.secret);
        console.log("Token: "+this.state.token+"; Secret: "+this.state.secret+"; Decrypted: "+decrypted);
        this.setState({ needToken: decrypted === '', decrypted: decrypted, redemptionKey: '' });
        event.preventDefault();
    }

    render() {
        return (
            <div>
                <div hidden={this.state.secret === undefined || this.state.secret === null}>
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
                </div>
                <div hidden={this.state.secret !== undefined && this.state.secret !== null}>
                    <form onSubmit={() => {this.loadSecret();}}>
                        <label>Key:</label>
                        <input name="key" type="text" onChange={this.handleChange}/>
                        <input type="submit" value="Submit" />
                    </form>
                </div>
            </div>
        );
    }
}

export default ViewSecret;