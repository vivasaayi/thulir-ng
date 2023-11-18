import { useState, useEffect } from 'react';

import { CButton, CCard, CCardBody, CCardHeader, CCol, CRow } from '@coreui/react'

import NetworkService from 'src/services/network-service';
import Post from './post';
import PostSummary from './post-summary';

function Posts() {
    const [posts, setPosts] = useState([]);
    const [postsLoaded, setPostsLoaded] = useState(false);

    async function initData() {
        if(postsLoaded) {
            return;
        }

        const url = "/api/posts";
        const result = await NetworkService.Get(url);
        setPostsLoaded(true);
        setPosts(result);
    }

    initData()

    function renderPosts() {
        const postsRendered = []
        posts.forEach(post => {
            postsRendered.push(
                <PostSummary post={post} />
            )
        })

        return postsRendered;
    }

    return (
        <>
            {renderPosts()}
        </>
    );
}

export default Posts;