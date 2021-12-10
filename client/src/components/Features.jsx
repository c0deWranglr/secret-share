import {Row, Col, Image} from 'react-bootstrap';

export function FeatureRow(props) {
    return (
        <Row xs={1} md={3} {...props} >
            {props.children}
        </Row>
    );
}

export function FeatureCol(props) {
    return (
        <Col className="featureCol" {...props} >
            <h2 className="pt-4">{props.title}</h2>
            {props.children}
        </Col>
    );
}

export function BlankCol(props) {
    return (<FeatureCol xs={0} md={1} />);
}

export function ImageCol(props) {
    return (
        <FeatureCol xs={5} md={4}>
            <Image src={props.src} />
        </FeatureCol>
    );
}

export function ListCol(props) {
    return (
        <FeatureCol title={props.title} md={6} style={{'background-color': 'var(--bs-gray-200)', 'border-radius': '10px'}} {...props}>
            <div className="d-flex justify-content-center">
                {props.children}
            </div>
        </FeatureCol>
    )
}