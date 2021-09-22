import React from 'react';
import { storeSecret } from '../lib/client';
import { encrypt } from '../lib/cryptography';

class SaveSecret extends React.Component {
    constructor(props) {
        super(props);
        this.state = { password: '', token: '', submitted: false, error: null, redemptionKey: '' };

        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.componentDidMount = this.componentDidMount.bind(this);
    }

    componentDidMount() {
        this.setState({ submitted: false, error: null, redemptionKey: '' });
    }

    handleChange(event) {
        let target = event.target;
        this.setState({ [target.name]: target.value });
    }

    handleSubmit(event) {
        this.setState( { error: null });
        
        let data = encrypt(this.state.token, this.state.password);
        storeSecret(data).then(key => {
            if (key !== undefined) {
                this.setState( { redemptionKey: key, submitted: true });
            } else {
                this.setState( { error: "Server unresponsive, please try again later." });
            }
        });
        event.preventDefault();
    }

    render() {
        return (
            <div>
                <div hidden={this.state.error == null}>
                    <label>{this.state.error}</label>
                    <br/>
                </div>
                <div hidden={this.state.submitted}>
                    <form onSubmit={this.handleSubmit}>
                        <label>Password:</label>
                        <input name="password" type="password" onChange={this.handleChange} />
                        <br/>
                        <label>Redemption Token:</label>
                        <input name="token" type="text" onChange={this.handleChange} />
                        <br/>
                        <input type="submit" value="Submit" />
                    </form>
                </div>
                <div hidden={!this.state.submitted}>
                    <label>Redemption Key: {this.state.redemptionKey}</label>
                    <br/>
                    <button onClick={this.componentDidMount}>Save Another</button>
                </div>
            </div>
        );
    }
}

export default SaveSecret;