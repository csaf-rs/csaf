<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:output method="text" />
    <xsl:template match="/">
        <xsl:for-each select="//*[local-name()='Weakness']">
            <xsl:value-of select="@ID" /><xsl:text>&#09;</xsl:text><xsl:value-of select="@Name" /><xsl:text>&#10;</xsl:text>  
        </xsl:for-each> 
    </xsl:template>
</xsl:stylesheet>
