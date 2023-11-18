import { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import {
    CButton,
    CCard,
    CCardBody,
    CCardHeader,
    CCol,
    CRow,
    CCardTitle,
    CCardImage,
    CCardText,
    CCardLink
} from '@coreui/react'

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
        if (post && post.title) {
            return (
                <>
                    <h1>{post.title.rendered}</h1>
                    <a href="#/dashboard">Back</a>
                </>
            );
        }

    }

    function renderPost() {
        if (post && post.content) {
            return (
                <>
                    <div dangerouslySetInnerHTML={{ __html: post.content.rendered }} />
                </>
            )
        }
    }

    return (
        <CCard>
            <CCardBody>
                <CCardTitle>{renderPostHeader()}</CCardTitle>
                <CCardText>
                    {renderPost()}
                </CCardText>
            </CCardBody>
        </CCard>
    );
}

export default Post;