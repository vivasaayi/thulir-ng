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

function PostSummary({ post }) {
    function renderLink(){
        const link = `#post/${post.id}`
        return <CCardLink href={link}>View</CCardLink>
    }
    
    function renderPostHeader() {
        const link = `#post/${post.id}`;
        return (
            <a href={link}>{post.title.rendered}</a>
        );
    }

    return (
        <div>
            <CCard>
                <CCardBody>
                    <CCardTitle>{renderPostHeader()}</CCardTitle>
                    <CCardText>
                        {post.excerpt.rendered}
                    </CCardText>
                </CCardBody>
                
            </CCard>
        </div>
    );
}

export default PostSummary;