import { useState, useEffect } from "react";
import { Link } from 'react-router-dom';
import HCaptcha from "@hcaptcha/react-hcaptcha";
import { Container, Stack, Form, Button } from "react-bootstrap";
import LogoHeading from "../components/LogoHeading";
import StepForm from "../components/StepForm";
import Step from "../components/Step";
import { resetInputs } from "../lib/inputUtils";
import { storeSecret } from '../lib/client';
import { encrypt } from '../lib/cryptography';

import "../style/formRoutes.css";

const initialState = {
    captcha: null,
    secret: '',
    token: '',
    expiration: null,
    maxAttempts: null,
    key: '',
    errorMessage: null,
};

export default function Save(props) {
    const [ state, setState ] = useState(initialState);
    var { secret, token, expiration, maxAttempts, key } = state;

    useEffect(() => {
        resetInputs();
    });

    const canEncrypt = () => secret && token;
    const validExpiration = () => expiration > 0;
    const update = (toUpdate) => setState({ ...state, ...toUpdate });

    return (
        <Stack>
            <LogoHeading>
                <h1>Save Secret</h1>
                <p>
                    Secrets are encrypted using a separate token in the browser before being stored on the server. The encrypted secret is stored by a randomly generated key.
                    <br/><br/>
                    To save a secret, enter the following:
                </p>
                <ul>
                    <li>Secret / Password - The sensitive information you want to share securely</li>
                    <li>Decrypt Token - The text used to decrypt the secret when time to view</li>
                    <li>Time To Live - How long the secret will be available for viewing</li>
                    <li>Max Attempts - The maximum number of times a request to view the secret can be made</li>
                </ul>
            </LogoHeading>
            <Container className="mb-4 view-form-container">
                <Container className={state.captcha && key ? "d-none" : "mt-4 d-flex justify-content-center"}>
                    <HCaptcha sitekey={window['config'].hCaptchaKey}
                              onVerify={(token) => update({ captcha: token }) } 
                              onExpire={() => update({ captcha: null })} 
                              onError={() => update({ captcha: null })} />
                </Container>
                <StepForm hidden={!state.captcha || key}
                          errorMessage={state.errorMessage}
                          canClear={false}>
                    <Step label="1. Encrypt the Secret"
                          buttonText="Encrypt"
                          buttonPlacement="bottom"
                          showButton={!canEncrypt()}
                          onButtonClick={() => update({ secret: secret, token: token })}>
                        <Form.Control placeholder="Secret / Password"
                                      type="password"
                                      plaintext={canEncrypt()}
                                      readOnly={canEncrypt()}
                                      defaultValue={secret}
                                      onChange={e => secret = e.target.value } />
                        <Form.Control placeholder="Encryption Text" 
                                      plaintext={canEncrypt()}
                                      readOnly={canEncrypt()}
                                      defaultValue={token}
                                      onChange={e => token = e.target.value }
                                      className="mt-2" />
                    </Step>
                    <Step label="2. Set Expiration"
                          hidden={!canEncrypt()}>
                        <Form.Control placeholder="Expiration in minutes"
                                      onChange={e => expiration = e.target.value }
                                      onBlur={() => update({ expiration: expiration }) }
                                      defaultValue={expiration}
                                      min={0}
                                      type="number" />
                        <Form.Control placeholder="Max Attempts (optional)"
                                      onChange={e => maxAttempts = e.target.value }
                                      onBlur={() => update({ maxAttempts: maxAttempts }) }
                                      defaultValue={maxAttempts}
                                      min={0}
                                      type="number"
                                      className="mt-2" />
                    </Step>
                    <Step hidden={!canEncrypt()}>
                        <div className="d-flex justify-content-end">
                            <Button type="submit"
                                    disabled={!validExpiration()} 
                                    onClick={() => encryptAndSave(state).then((key) => {
                                        if (!key) { update({ errorMessage: 'Error saving secret. Please try again.' })}
                                        else update({ key: key, errorMessage: null });
                                    })}>
                                Save
                            </Button>
                        </div>
                    </Step>
                </StepForm>
                <Container className={state.captcha && key ? "mt-4 d-flex justify-content-center" : "d-none"}>
                    <h3>
                        Access Key:
                        <br/>
                        <Link to={"/view?key="+key}>
                            {key}
                        </Link>
                    </h3>
                </Container>
            </Container>
        </Stack>
    );
};

async function encryptAndSave({ captcha, secret, token, expiration, maxAttempts }) {
    const encrypted = encrypt(token, secret);
    const key = await storeSecret(captcha, encrypted, expiration, maxAttempts);
    return Promise.resolve(key);
};