import '../style/logoHeading.css';

export default function LogoHeading(props) {
    return (
        <div className="logoHeadingBackground">
            <div className="logoHeading">
                {props.children}
            </div>
        </div>
    );
}