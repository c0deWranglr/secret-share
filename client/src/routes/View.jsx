import { useState, useEffect } from "react";
import { Container, Form, Row, Stack } from "react-bootstrap";
import LogoHeading from "../components/LogoHeading";
import Step from "../components/Step";
import StepForm from "../components/StepForm";
import HCaptcha from '@hcaptcha/react-hcaptcha';
import { getSecret } from '../lib/client';
import { decrypt } from '../lib/cryptography';
import { resetInputs } from "../lib/inputUtils";

import "../style/formRoutes.css";

const initialState = {
    captcha: null,
    key: '',
    token: '',
    secret: '',
    errorMessage: null
};

export default function View(props) {
    const [ state, setState ] = useState(initialState);
    var {key, token, secret} = state;

    useEffect(() => {
        maybeSetUrlKey(props, update)
        resetInputs();
    });

    const update = (toUpdate) => setState({ ...state, ...toUpdate });
    const validKey = () => key && secret;
    const validToken = () => token && true;

    return (
        <Stack>
            <LogoHeading>
                <h1>View Secret</h1>
                <p>
                    Secrets are encrypted using a separate token in the browser before being stored on the server. The encrypted secret is stored by a randomly generated key.
                    <br/><br/>
                    To view a secret, enter the following:
                </p>
                <ul>
                    <li>Access Key - Location the secret is stored server-side</li>
                    <li>Decrypt Token - The user specified token used at time of secret saving</li>
                </ul>
            </LogoHeading>
            <Container className="mb-4 view-form-container">
                <Container className="mt-4 d-flex justify-content-center">
                    <HCaptcha sitekey={window['config'].hCaptchaKey}
                              onVerify={(token) => update({ captcha: token })} 
                              onExpire={() => update({ captcha: null })} 
                              onError={() => update({ captcha: null })} />
                </Container>
                <StepForm hidden={!state.captcha} 
                          errorMessage={state.errorMessage}
                          canClear={validKey()} 
                          clearState={() => setState({ ...initialState, captcha: state.captcha })}>
                    <Step label="1. Access Key" 
                          buttonText="Load"
                          showButton={!validKey()}
                          onButtonClick={() => loadSecret(state.captcha, key).then(secret => { 
                              if (!secret) update({ errorMessage: 'Error loading secret. Please enter a valid access key.' });
                              else update({ key: key, secret: secret, errorMessage: null });
                          })}>
                        <Form.Control placeholder="GXy2" 
                                      defaultValue={key} 
                                      plaintext={validKey()} 
                                      readOnly={validKey()} 
                                      onChange={e => key = e.target.value } />
                    </Step>
                    <Step hidden={!validKey()} 
                          label="2. Decrypt Token" 
                          buttonText="Decrypt"
                          showButton={!validToken()}
                          onButtonClick={() => {
                              const decrypted = decrypt(token, secret);
                              if (!decrypted) update({ errorMessage: 'The token you entered is incorrect. Please enter the valid token.'})
                              else update({ token: token, errorMessage: null });
                          }}>
                        <Form.Control placeholder="12345" 
                                      defaultValue={token}
                                      plaintext={validToken()} 
                                      readOnly={validToken()} 
                                      onChange={e => token = e.target.value } />
                    </Step>
                    <Row className="mt-3 text-center" 
                         hidden={!validToken()}>
                        <textarea disabled value={validKey() && validToken() ? decrypt(token, secret) : ''} />
                    </Row>
                </StepForm>
            </Container>
        </Stack>
    );
}

async function loadSecret(captcha, key) {
   if (key) {
       const secret = await getSecret(captcha, key);
       if (secret) {
           return Promise.resolve(secret);
       }
   }
   return Promise.resolve('');
}

function maybeSetUrlKey(props, update) {
    const urlKey = extractKey(props.location);
    if (urlKey) {
        props.history.push(document.location.pathname);
        update({ key: urlKey });
    }
}

function extractKey(location) {
    let params = location.search;
    let start = params.indexOf("key=")+4;
    if (start <= 3) { return ""; } //Doesn't contain 'key' param
    var end = params.indexOf("&", start);
    if (end < 0) { end = params.length; }
    return params.substring(start, end);
}
