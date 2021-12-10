import {Stack, Container} from 'react-bootstrap';
import {FeatureRow, BlankCol, ImageCol, ListCol} from '../components/Features';
import LinkButton from '../components/LinkButton';
import LogoHeading from '../components/LogoHeading';
import '../style/home.css';


export default function Home(props) {
    return (
    <Stack gap={3}>
        <LogoHeading>
            <Container className="text-center text-md-start">
                <h1 className="homeHeader">{props.name}</h1>
                <p className="homeDesc">Your secret is safe here. Share confidently.</p>
                <Stack className="homeActions justify-content-center justify-content-md-start" direction="horizontal" gap={1}>
                    <LinkButton to="/save" variant="outline-primary" size="lg">Save a Secret</LinkButton>
                    <LinkButton to="/view" variant="outline-secondary" size="lg">View a Secret</LinkButton>
                </Stack>
            </Container>
        </LogoHeading>
        <Container className="mb-3">
            <FeatureRow className="justify-content-center">
                <BlankCol />
                <ImageCol src="/assets/key.svg" />
                <BlankCol /> 
                <ListCol title="Security First">
                    <ul className="mt-md-4">
                        <li>Secrets are AES 256 encrypted in the local browser</li>
                        <li>Secrets are encrypted using a user specified a 1000x hashed token</li>
                        <li>Secrets are encrypted a second time server side</li>
                        <li>Open source to ensure public accountability</li>
                    </ul>
                </ListCol>
            </FeatureRow>
            <FeatureRow className="mt-3 justify-content-center">
                <ListCol title="3 Easy Steps" xs={{order: 1}} md={{order: 0, span:6}}>
                    <ul className="mt-md-3">
                        <li className="numbered">Save a secret, deciding on an access token and expiration</li>
                        <li className="numbered">Share randomly generated access key link</li>
                        <li className="numbered">Recipient enters your token to view password</li>
                    </ul>
                </ListCol>
                <BlankCol />
                <ImageCol src="/assets/handshake.svg" />
                <BlankCol />
            </FeatureRow>
        </Container>
    </Stack>
    );
}