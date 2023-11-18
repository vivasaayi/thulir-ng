import { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import NetworkService from 'src/services/network-service';

function Post({ postId }) {
    let params = useParams();

    const [post, setPost] = useState({});
    const [postLoaded, setPostLoaded] = useState(false);

    async function initPost() {
        if (postLoaded) {
            return
        }

        const url = `/api/post/${params.postId}`;
        const result = await NetworkService.Get(url);
        setPostLoaded(true);
        setPost(result);
    }

    initPost()

    function renderPostHeader() {
        return (
            <h1>{post.title.rendered}</h1>
        );
    }

    function renderPost() {
        if (post && post.content) {
            return (
                <>
                    {renderPostHeader()}
                    <div dangerouslySetInnerHTML={{ __html: post.content.rendered }} />
                </>
            )
        }
    }

    return (
        <div>
            {renderPost()}
        </div>
    );
}

export default Post;