import { useParams } from 'react-router-dom';

function Post({postId}) {
    let params = useParams();

    return (
        <div>
            Hello
            <pre>{JSON.stringify(params.postId)}</pre>
        </div>
    );
}

export default Post;