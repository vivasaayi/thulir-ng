import { useState, useEffect } from 'react';

import { CButton, CCard, CCardBody, CCardHeader, CCol, CRow } from '@coreui/react'

import NetworkService from 'src/services/network-service';
import Post from './post';
import PostSummary from './post-summary';

function Posts() {
    const [posts, setPosts] = useState([]);

    async function initData() {
        const url = "/api/posts";
        const result = await NetworkService.Get(url);
        setPosts(result);
    }

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
            <CButton onClick={initData}>
                Fetch Post
            </CButton>
            {renderPosts()}
        </>
    );
}

export default Posts;